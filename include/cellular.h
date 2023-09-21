#include <mutex>

#define API_ROOT "http://192.168.0.1"
#define ADMIN_PASSWORD "YWRtaW4%3D"

/*
Singleton cellular class, controls connection related stuff.

Functions:
private:
async fn set_cmd_process(func: &str, field: &str, value: &str) ->
Result<Response, Error> { let client = reqwest::Client::new();
    client.post(API_ROOT.to_string() + "/goform/goform_set_cmd_process")
        .body(format!("goformId={func}&{field}={value}"))
        .timeout(Duration::from_secs(2))
        .send()
        .await
}

async fn get_cmd_process(fields: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let url =
format!("/goform/goform_get_cmd_process?multi_data=1&sms_received_flag_flag=0&sts_received_flag_flag=0&cmd={fields}");
    client.get(API_ROOT.to_string() +
&url).timeout(Duration::from_secs(2)).send().await
}

*/

/* @see cellular.cc */
class cellular {
  
};