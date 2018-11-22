package com.filebash.api

import com.typesafe.config.Config
import io.rdbc.pgsql.core.config.sapi.Auth
import io.rdbc.pgsql.transport.netty.sapi.NettyPgConnectionFactory
import io.rdbc.pool.sapi.{ConnectionPool, ConnectionPoolConfig}
import io.rdbc.sapi.{Connection, Timeout}

import scala.concurrent.{ExecutionContext, Future}

class Store(config: Config) {
  implicit val executionContext: ExecutionContext = NiceExecutionContext("store")
  private val connectionFactory = NettyPgConnectionFactory(NettyPgConnectionFactory.Config(
    host = config.getString("host"),
    port = config.getInt("port"),
    authenticator = Auth.password(config.getString("user"), config.getString("pass")),
    dbName = Some(config.getString("base")),
    ec = executionContext
  ))
  private val connectionPool = ConnectionPool(connectionFactory, ConnectionPoolConfig(size = 32, ec = executionContext))

  def withConnection[A](action: Connection => Future[A])(implicit timeout: Timeout = Timeout.Inf): Future[A] =
    connectionPool.withConnection(action)
}
