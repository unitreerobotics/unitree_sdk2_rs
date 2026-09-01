#pragma once
#include "rust/cxx.h"
#include <memory>
#include <string>

namespace unitree {

struct RpcRequestHandler;

class RpcClient {
public:
  explicit RpcClient(std::string service);
  ~RpcClient();
  void register_api(int32_t api_id) const;
  rust::String call(int32_t api_id, rust::Str request) const;
private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
};
auto new_rpc_client(rust::Str service) -> std::unique_ptr<RpcClient>;

class RpcServer {
public:
  explicit RpcServer(std::string service);
  ~RpcServer();
  void start() const;
  void register_handler(int32_t api_id, rust::Box<RpcRequestHandler> handler) const;
private:
  struct Impl;
  std::unique_ptr<Impl> impl_;
};
auto new_rpc_server(rust::Str service) -> std::unique_ptr<RpcServer>;

} // namespace unitree
