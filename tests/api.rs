//! Cloud art-QR generation (feature `api`): orchestration, retry/verify, the
//! Replicate provider logic (mock HTTP), and the real `UreqClient` (localhost).
#![cfg(feature = "api")]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::rc::Rc;
use std::thread;

use qrc::api::replicate::{HttpClient, HttpResponse, ReplicateProvider, UreqClient};
use qrc::api::{generate, ArtRequest, Provider, RetryOptions};
use qrc::encode::QrOptions;
use qrc::error::QrError;
use qrc::render::control::ControlOptions;
use qrc::render::raster::RasterOptions;
use qrc::QRCode;

// --- helpers ---------------------------------------------------------------

/// A real, scannable QR PNG.
fn qr_png(payload: &str) -> Vec<u8> {
    QRCode::from_string(payload.to_string())
        .to_png_bytes(&QrOptions::new(), &RasterOptions::default())
        .unwrap()
}

/// A valid PNG that is not a QR code.
fn plain_png() -> Vec<u8> {
    let img = image::RgbaImage::from_pixel(16, 16, image::Rgba([90, 140, 200, 255]));
    qrc::render::raster::image_to_bytes(&img, image::ImageFormat::Png).unwrap()
}

fn small_qr() -> QRCode {
    QRCode::from_string("https://example.com".to_string())
}

/// A `Provider` yielding a scripted sequence of results.
struct MockProvider {
    results: RefCell<VecDeque<Result<Vec<u8>, QrError>>>,
}
impl MockProvider {
    fn new(results: Vec<Result<Vec<u8>, QrError>>) -> Self {
        MockProvider {
            results: RefCell::new(results.into()),
        }
    }
}
impl Provider for MockProvider {
    fn generate(&self, _control: &[u8], _req: &ArtRequest) -> Result<Vec<u8>, QrError> {
        self.results
            .borrow_mut()
            .pop_front()
            .expect("ran out of mock results")
    }
}

/// An `HttpClient` returning scripted responses and recording request bodies.
/// Cloneable (shared state) so a test can inspect captures after the provider
/// has taken ownership of its copy.
#[derive(Clone)]
struct MockHttp {
    responses: Rc<RefCell<VecDeque<Result<HttpResponse, QrError>>>>,
    bodies: Rc<RefCell<Vec<Vec<u8>>>>,
}
impl MockHttp {
    fn new(responses: Vec<Result<HttpResponse, QrError>>) -> Self {
        MockHttp {
            responses: Rc::new(RefCell::new(responses.into())),
            bodies: Rc::new(RefCell::new(Vec::new())),
        }
    }
}
impl HttpClient for MockHttp {
    fn send(
        &self,
        _method: &str,
        _url: &str,
        _headers: &[(&str, &str)],
        body: Option<&[u8]>,
    ) -> Result<HttpResponse, QrError> {
        if let Some(b) = body {
            self.bodies.borrow_mut().push(b.to_vec());
        }
        self.responses
            .borrow_mut()
            .pop_front()
            .expect("ran out of mock responses")
    }
}

fn ok(status: u16, body: &[u8]) -> Result<HttpResponse, QrError> {
    Ok(HttpResponse {
        status,
        body: body.to_vec(),
    })
}

// --- ArtRequest / RetryOptions ---------------------------------------------

#[test]
fn art_request_builder() {
    let r = ArtRequest::new("a koi pond")
        .negative_prompt("text")
        .conditioning_scale(2.0)
        .qr_payload("https://example.com");
    assert_eq!(r.prompt, "a koi pond");
    assert_eq!(r.negative_prompt, "text");
    assert!((r.conditioning_scale - 2.0).abs() < f32::EPSILON);
    assert_eq!(r.qr_payload.as_deref(), Some("https://example.com"));
    assert_eq!(RetryOptions::default().max_attempts, 3);
}

// --- orchestration: generate / to_ai_art -----------------------------------

#[test]
fn generate_returns_first_scannable_result() {
    let provider = MockProvider::new(vec![Ok(qr_png("https://example.com/ai"))]);
    let out = generate(
        &small_qr(),
        &QrOptions::new(),
        &ControlOptions::with_size(120),
        &provider,
        &ArtRequest::new("art"),
        &RetryOptions::default(),
    )
    .unwrap();
    assert!(!out.is_empty());
}

#[test]
fn generate_retries_until_scannable() {
    let provider = MockProvider::new(vec![Ok(plain_png()), Ok(qr_png("retry"))]);
    let out = small_qr()
        .to_ai_art(
            &QrOptions::new(),
            &ControlOptions::with_size(120),
            &provider,
            &ArtRequest::new("art"),
            &RetryOptions::default(),
        )
        .unwrap();
    assert!(!out.is_empty());
}

#[test]
fn generate_gives_up_after_retries_on_unscannable() {
    let provider = MockProvider::new(vec![Ok(plain_png()), Ok(b"not an image".to_vec())]);
    let err = generate(
        &small_qr(),
        &QrOptions::new(),
        &ControlOptions::with_size(120),
        &provider,
        &ArtRequest::new("art"),
        &RetryOptions { max_attempts: 2 },
    )
    .unwrap_err();
    assert!(matches!(err, QrError::Api(_)));
}

#[test]
fn generate_surfaces_provider_errors_and_clamps_zero_attempts() {
    let provider = MockProvider::new(vec![Err(QrError::Api("boom".into()))]);
    let err = generate(
        &small_qr(),
        &QrOptions::new(),
        &ControlOptions::with_size(120),
        &provider,
        &ArtRequest::new("art"),
        &RetryOptions { max_attempts: 0 }, // clamped to 1 attempt
    )
    .unwrap_err();
    assert!(matches!(err, QrError::Api(_)));
}

#[test]
fn generate_propagates_encode_errors() {
    let provider = MockProvider::new(vec![]); // never called
    let err = QRCode::from_string("Z".repeat(8000))
        .to_ai_art(
            &QrOptions::new(),
            &ControlOptions::default(),
            &provider,
            &ArtRequest::new("art"),
            &RetryOptions::default(),
        )
        .unwrap_err();
    assert!(matches!(err, QrError::DataTooLong | QrError::Encode(_)));
}

// --- Replicate provider logic (mock HTTP) ----------------------------------

#[test]
fn replicate_happy_path_builds_request_and_returns_image() {
    let png = qr_png("replicate");
    let http = MockHttp::new(vec![
        ok(
            200,
            br#"{"status":"succeeded","output":"http://host/out.png"}"#,
        ),
        ok(200, &png),
    ]);
    let spy = http.clone();
    let provider = ReplicateProvider::with_client(http, "tok", "owner/model:ver", "http://host");
    let out = provider
        .generate(b"CONTROL", &ArtRequest::new("a forest").qr_payload("hi"))
        .unwrap();
    assert_eq!(out, png);

    // The POST body carries the prompt, the base64 data URI and the payload.
    let body = String::from_utf8(spy.bodies.borrow()[0].clone()).unwrap();
    assert!(body.contains("\"prompt\":\"a forest\""));
    assert!(body.contains("data:image/png;base64,"));
    assert!(body.contains("qr_code_content"));
}

#[test]
fn replicate_omits_payload_when_absent() {
    let http = MockHttp::new(vec![
        ok(200, br#"{"output":"http://host/o"}"#),
        ok(200, b"img"),
    ]);
    let spy = http.clone();
    let provider = ReplicateProvider::with_client(http, "tok", "ver", "http://host");
    provider
        .generate(b"PNG", &ArtRequest::new("sunset"))
        .unwrap();
    let body = String::from_utf8(spy.bodies.borrow()[0].clone()).unwrap();
    assert!(!body.contains("qr_code_content"));
}

#[test]
fn replicate_reports_http_errors() {
    let http = MockHttp::new(vec![ok(401, b"unauthorized")]);
    let provider = ReplicateProvider::with_client(http, "tok", "ver", "http://host");
    let err = provider.generate(b"x", &ArtRequest::new("p")).unwrap_err();
    assert!(matches!(err, QrError::Api(m) if m.contains("HTTP 401")));

    let http = MockHttp::new(vec![
        ok(200, br#"{"output":"http://host/o"}"#),
        ok(404, b""),
    ]);
    let provider = ReplicateProvider::with_client(http, "tok", "ver", "http://host");
    let err = provider.generate(b"x", &ArtRequest::new("p")).unwrap_err();
    assert!(matches!(err, QrError::Api(m) if m.contains("HTTP 404")));

    // POST succeeds but fetching the output image transport-fails.
    let http = MockHttp::new(vec![
        ok(200, br#"{"output":"http://host/o"}"#),
        Err(QrError::Api("GET exploded".into())),
    ]);
    let provider = ReplicateProvider::with_client(http, "tok", "ver", "http://host");
    let err = provider.generate(b"x", &ArtRequest::new("p")).unwrap_err();
    assert!(matches!(err, QrError::Api(m) if m.contains("GET exploded")));
}

#[test]
fn replicate_parses_output_variants_and_errors() {
    let cases: Vec<(&[u8], bool)> = vec![
        (br#"{"output":["http://host/a.png"]}"#, true), // array form -> ok
        (br#"{"status":"failed","error":"NSFW"}"#, false),
        (br#"{"status":"canceled"}"#, false),
        (br#"{"output":[]}"#, false),
        (br#"{"foo":"bar"}"#, false),
        (b"not json", false),
    ];
    for (body, should_ok) in cases {
        let http = MockHttp::new(vec![ok(200, body), ok(200, b"img")]);
        let provider = ReplicateProvider::with_client(http, "t", "v", "http://h");
        let result = provider.generate(b"x", &ArtRequest::new("p"));
        assert_eq!(
            result.is_ok(),
            should_ok,
            "body: {}",
            String::from_utf8_lossy(body)
        );
    }
}

#[test]
fn http_response_and_clients_derive_traits() {
    let r = HttpResponse {
        status: 200,
        body: vec![1, 2],
    };
    assert_eq!(r.clone(), r);
    assert!(format!("{r:?}").contains("200"));
    let _ = UreqClient::new();
    let p = ReplicateProvider::new("tok", "ver");
    assert!(format!("{p:?}").contains("ver"));
}

// --- real UreqClient over localhost ----------------------------------------

/// Spawns a server that answers each scripted `(status, body)` on its own
/// connection, in order. Returns the base URL.
fn serve(responses: Vec<(u16, Vec<u8>)>) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    thread::spawn(move || {
        for (status, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 8192];
            let _ = stream.read(&mut buf);
            let head = format!(
                "HTTP/1.1 {status} X\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(head.as_bytes());
            let _ = stream.write_all(&body);
            let _ = stream.flush();
        }
    });
    base
}

#[test]
fn ureq_client_real_round_trip() {
    // Bind first so the prediction's output URL can point back at this server.
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let prediction = format!(r#"{{"status":"succeeded","output":["{base}/img"]}}"#);
    thread::spawn(move || {
        let responses = [
            (201u16, prediction.into_bytes()),
            (200u16, b"FAKE-IMAGE-BYTES".to_vec()),
        ];
        for (status, body) in responses {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 8192];
            let _ = stream.read(&mut buf);
            let head = format!(
                "HTTP/1.1 {status} X\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(head.as_bytes());
            let _ = stream.write_all(&body);
            let _ = stream.flush();
        }
    });

    let provider = ReplicateProvider::with_client(UreqClient::new(), "tok", "ver", &base);
    let out = provider.generate(b"tiny", &ArtRequest::new("p")).unwrap();
    assert_eq!(out, b"FAKE-IMAGE-BYTES");
}

#[test]
fn ureq_client_get_failure_after_successful_post() {
    // POST succeeds with an output URL whose host is dead, so the GET fails at
    // the transport level — covering that path for the real `UreqClient`.
    let dead = TcpListener::bind("127.0.0.1:0").unwrap();
    let dead_url = format!("http://{}/img", dead.local_addr().unwrap());
    drop(dead);
    let base = serve(vec![(
        201,
        format!(r#"{{"output":["{dead_url}"]}}"#).into_bytes(),
    )]);
    let provider = ReplicateProvider::with_client(UreqClient::new(), "t", "v", &base);
    let err = provider.generate(b"x", &ArtRequest::new("p")).unwrap_err();
    assert!(matches!(err, QrError::Api(m) if m.contains("transport")));
}

#[test]
fn ureq_client_maps_status_and_transport_errors() {
    // 5xx -> surfaced as an HTTP error (covers the ureq Status arm).
    let base = serve(vec![(500, b"server error".to_vec())]);
    let provider = ReplicateProvider::with_client(UreqClient::new(), "tok", "ver", &base);
    let err = provider.generate(b"x", &ArtRequest::new("p")).unwrap_err();
    assert!(matches!(err, QrError::Api(m) if m.contains("HTTP 500")));

    // Connection refused -> transport error.
    let dead = TcpListener::bind("127.0.0.1:0").unwrap();
    let dead_base = format!("http://{}", dead.local_addr().unwrap());
    drop(dead);
    let provider = ReplicateProvider::with_client(UreqClient::new(), "tok", "ver", &dead_base);
    let err = provider.generate(b"x", &ArtRequest::new("p")).unwrap_err();
    assert!(matches!(err, QrError::Api(m) if m.contains("transport")));
}
