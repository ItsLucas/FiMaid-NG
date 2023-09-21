#include <boost/asio/connect.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/strand.hpp>
#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/version.hpp>
#include <iostream>
#include <string>

namespace beast = boost::beast;
namespace http = beast::http;
namespace net = boost::asio;
using tcp = net::ip::tcp;

template <typename Body, typename Allocator, typename Send>
void send_request(http::request<Body, http::basic_fields<Allocator>> &&req,
                  Send &&send) {
  net::io_context ioc;
  tcp::resolver resolver(ioc);
  beast::tcp_stream stream(ioc);
  auto const results = resolver.resolve("192.168.0.1", "80");
  stream.connect(results);

  http::write(stream, req);

  beast::flat_buffer buffer;
  http::response<http::dynamic_body> res;
  http::read(stream, buffer, res);

  send(std::move(res));
}

template <typename Send>
void set_cmd_process(const std::string &func, const std::string &field,
                     const std::string &value, Send &&send) {
  http::request<http::string_body> req{http::verb::post,
                                       "/goform/goform_set_cmd_process", 11};
  req.set(http::field::host, "192.168.0.1");
  req.set(http::field::user_agent, BOOST_BEAST_VERSION_STRING);
  req.set(http::field::content_type, "application/x-www-form-urlencoded");
  req.body() = "goformId=" + func + "&" + field + "=" + value;
  req.prepare_payload();

  send_request(std::move(req), std::forward<Send>(send));
}

template <typename Send>
void get_cmd_process(const std::string &fields, Send &&send) {
  http::request<http::string_body> req{
      http::verb::get,
      "/goform/"
      "goform_get_cmd_process?multi_data=1&sms_received_flag_flag=0&sts_"
      "received_flag_flag=0&cmd=" +
          fields,
      11};
  req.set(http::field::host, "192.168.0.1");
  req.set(http::field::user_agent, BOOST_BEAST_VERSION_STRING);
  send_request(std::move(req), std::forward<Send>(send));
}