#include <csignal>
#include <cstdio>
#include <cstdlib>
#include <exception>
#include <string>

#include <tgbot/net/HttpClient.h>
#include <tgbot/tgbot.h>

using namespace std;
using namespace TgBot;

std::unique_ptr<Bot> bot;
HttpClient &_getDefaultHttpClient() {
  static BoostHttpOnlySslClient instance;
  return instance;
}

int main() {
  std::string token(getenv("TOKEN"));
  bot = std::make_unique<Bot>(token, _getDefaultHttpClient(), "http://raburaibu.me0w.men");

  bot->getEvents().onCommand("start", [](Message::Ptr message) {
    bot->getApi().sendMessage(message->chat->id, "Hi!");
  });

  return 0;
}