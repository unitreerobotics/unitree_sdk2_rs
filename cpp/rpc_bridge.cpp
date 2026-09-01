#include "unitree_sdk2_rs/src/lib.rs.h"
#include "rpc_bridge.h"
#include <unitree/robot/channel/channel_factory.hpp>
#include <unitree/robot/client/client.hpp>
#include <unitree/robot/server/server.hpp>
#include <sstream>
#include <vector>

namespace unitree {

namespace {

class RpcClientImpl : public unitree::robot::Client {
  std::vector<int32_t> apis_;
  bool inited_ = false;
public:
  RpcClientImpl(const std::string &svc) : Client(svc, false) {}
  void Init() override { for (auto id : apis_) RegistApi(id); SetTimeout(20.0f); }
  void add_api(int32_t id) { apis_.push_back(id); }
  void ensure_init() { if (!inited_) { Init(); inited_ = true; } }
  static std::string esc(const std::string &s) {
      std::string out;
      for (char c : s) {
          switch (c) {
          case '"': out += "\\\""; break;
          case '\\': out += "\\\\"; break;
          case '\n': out += "\\n"; break;
          case '\r': out += "\\r"; break;
          case '\t': out += "\\t"; break;
          default: out += c;
          }
      }
      return out;
  }
  rust::String call_api(int32_t api_id, const std::string &req) {
      ensure_init();
      std::string rsp;
      int32_t code = Call(api_id, req, rsp);
      std::ostringstream os;
      os << R"({"code":)" << code << R"(,"data":")" << esc(rsp) << R"("})";
      return rust::String(os.str());
  }
};

class RpcServerImpl : public unitree::robot::Server {
public:
  RpcServerImpl(const std::string &svc) : Server(svc) {}
  void Init() override {}
  void reg(int32_t id, const unitree::robot::RequestHandler &h, bool lease) { RegistHandler(id, h, lease); }
};

} // namespace

struct RpcClient::Impl {
  std::unique_ptr<RpcClientImpl> client;
  Impl(const std::string &s) : client(std::make_unique<RpcClientImpl>(s)) {}
};

RpcClient::RpcClient(std::string service) : impl_(std::make_unique<Impl>(service)) {}
RpcClient::~RpcClient() = default;
void RpcClient::register_api(int32_t api_id) const { impl_->client->add_api(api_id); }
rust::String RpcClient::call(int32_t api_id, rust::Str request) const {
  return impl_->client->call_api(api_id, std::string(request));
}
auto new_rpc_client(rust::Str service) -> std::unique_ptr<RpcClient> {
  return std::make_unique<RpcClient>(std::string(service));
}

struct RpcServer::Impl {
  std::unique_ptr<RpcServerImpl> server;
  Impl(const std::string &s) : server(std::make_unique<RpcServerImpl>(s)) { server->Init(); }
};

RpcServer::RpcServer(std::string service) : impl_(std::make_unique<Impl>(service)) {}
RpcServer::~RpcServer() = default;
void RpcServer::start() const { impl_->server->Start(); }
void RpcServer::register_handler(int32_t api_id, rust::Box<RpcRequestHandler> handler) const {
  auto h = std::make_shared<rust::Box<RpcRequestHandler>>(std::move(handler));
  impl_->server->reg(api_id, [h](const std::string &req, std::string &rsp) -> int32_t {
      rsp = std::string((*h)->handle(rust::Str(req)));
      return 0;
  }, false);
}
auto new_rpc_server(rust::Str service) -> std::unique_ptr<RpcServer> {
  return std::make_unique<RpcServer>(std::string(service));
}

} // namespace unitree
