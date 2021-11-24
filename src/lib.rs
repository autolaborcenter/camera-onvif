pub fn build_ptz(ptz: (f32, f32, f32)) -> String {
    let body = build_ptz_body(0, ptz);
    format!(
        "\
POST / HTTP/1.1\r\n\
Accept:application/soap+xml, text/html, image/gif, image/jpeg, *; q=.2, */*; q=.2\r\n\
Content-Type:application/soap+xml; charset=utf-8\r\n\
Content-Length:{}\r\n\
\r\n\
{}",
        body.len(),
        body
    )
}

pub fn build_reset() -> String {
    let body0 = build_get_preset_body(0);
    let body1 = build_goto_preset_body(1);
    format!(
        "\
POST / HTTP/1.1\r\n\
Accept:application/soap+xml, text/html, image/gif, image/jpeg, *; q=.2, */*; q=.2\r\n\
Content-Type:application/soap+xml; charset=utf-8\r\n\
Content-Length:{}\r\n\
\r\n\
{}\
POST / HTTP/1.1\r\n\
Accept:application/soap+xml, text/html, image/gif, image/jpeg, *; q=.2, */*; q=.2\r\n\
Content-Type:application/soap+xml; charset=utf-8\r\n\
Content-Length:{}\r\n\
\r\n\
{}
",
        body0.len(),
        body0,
        body1.len(),
        body1
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

fn build_get_preset_body(nonce: i32) -> String {
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
        <GetPresets xmlns=\"http://www.onvif.org/ver20/ptz/wsdl\" xmlns:ns2=\"http://www.onvif.org/ver10/schema\">
            <ProfileToken>MainStreamProfileToken</ProfileToken>
        </GetPresets>
    </env:Body>
</env:Envelope>", user_name, base64::encode(password), base64::encode(nonce), time_stamp)
}

fn build_goto_preset_body(nonce: i32) -> String {
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
        <GotoPreset xmlns=\"http://www.onvif.org/ver20/ptz/wsdl\" xmlns:ns2=\"http://www.onvif.org/ver10/schema\">
            <ProfileToken>MainStreamProfileToken</ProfileToken>
            <PresetToken>Preset001</PresetToken>
            <Speed>
                <ns2:PanTilt x=\"1.0\" y=\"1.0\"/>
                <ns2:Zoom x=\"1.0\"/>
            </Speed>
        </GotoPreset>
    </env:Body>
</env:Envelope>", user_name, base64::encode(password), base64::encode(nonce), time_stamp)
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
