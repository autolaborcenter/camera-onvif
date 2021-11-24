use std::net::IpAddr;

pub fn build_gstreamer(address: IpAddr, stream: u8) -> String {
    format!(
        "\
rtspsrc location=rtsp://admin:admin@192.168.188.88:554/{} \
! application/x-rtp,media=video \
! udpsink host={} port=5006 sync=false",
        stream, address
    )
}

pub fn build_ptz(ptz: (f32, f32, f32)) -> String {
    let body = build_ptz_body(0, ptz);
    format!(
        "\
GET / HTTP/1.1\r\n\
soapUri:http://192.168.1.88:8080/onvif/ptz_service\r\n\
Accept:application/soap+xml, text/html, image/gif, image/jpeg, *; q=.2, */*; q=.2\r\n\
Content-Type:application/soap+xml; charset=utf-8\r\n\
Content-Length:{}\r\n\
\r\n\
{}",
        body.len(),
        body
    )
}

fn build_ptz_body(nonce: i32, ptz: (f32, f32, f32)) -> String {
    let time_stamp = get_utc();
    let nonce = nonce.to_string();
    let user_name = "admin";
    let password = sha1::Sha1::from(format!("{}{}{}", nonce, time_stamp, "admin"))
        .digest()
        .to_string();

    format!("\
<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>
<env:Envelope xmlns:env=\"http://www.w3.org/2003/05/soap-envelope\" xmlns:wsse=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd\" xmlns:wsu=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-utility-1.0.xsd\">
    <env:Header>
        <wsse:Security>
            <wsse:UsernameToken>
                <wsse:Username>{}</wsse:Username>
                <wsse:Password Type=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-username-token-profile-1.0#PasswordDigest\">{}</wsse:Password>
                <wsse:Nonce EncodingType=\"http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-soap-message-security-1.0#Base64Binary\">{}</wsse:Nonce>
                <wsu:Created>{}</wsu:Created>
            </wsse:UsernameToken>
        </wsse:Security>
    </env:Header>
    <env:Body>
        <ContinuousMove xmlns=\"http://www.onvif.org/ver20/ptz/wsdl\" xmlns:ns2=\"http://www.onvif.org/ver10/schema\">
            <ProfileToken>MainStreamProfileToken</ProfileToken>
            <Velocity>
                <ns2:PanTilt x=\"{}\" y=\"{}\"/>
                <ns2:Zoom x=\"{}\"/>
            </Velocity>
        </ContinuousMove>
    </env:Body>
</env:Envelope>", user_name, base64::encode(password), base64::encode(nonce), time_stamp, ptz.0, ptz.1, ptz.2)
}

fn get_utc() -> String {
    use chrono::{DateTime, Datelike, Utc};
    let utc: DateTime<Utc> = Utc::now();
    format!(
        "{:04}-{:02}-{}T{}Z",
        utc.year(),
        utc.month(),
        utc.day(),
        utc.format("%H:%M:%S")
    )
}
