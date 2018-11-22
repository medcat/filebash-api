package com.filebash.api.http

import akka.actor.ActorSystem
import akka.http.scaladsl.server.{HttpApp, Route}
import com.filebash.api.Store
import com.filebash.api.http.routes.SessionRoute
import com.typesafe.config.ConfigFactory
import com.typesafe.scalalogging.StrictLogging

object Api extends StrictLogging {
  def main(_args: Array[String]): Unit = {
    logger.warn("starting up api server...")
    val config = ConfigFactory.load
    val host = config.getString("com.petametrics.api.host")
    val port = config.getInt("com.filebash.api.port")
    implicit val actorSystem: ActorSystem = ActorSystem("api-server", config)

    val store = new Store(config.getConfig("com.filebash.store"))

    val httpApi = new Api(store)

    httpApi.startServer(host, port, actorSystem)
  }
}

class Api(store: Store) extends HttpApp {
  override protected def routes: Route =
    (decodeRequest & encodeResponse) {
      sessionRoute ~ userRoute ~ inviteRoute
    }

  private def sessionRoute =
    pathPrefix("session") {
      (path("create") & post) {SessionRoute.create(store)}
    }

  private def userRoute = ???

  private def inviteRoute = ???
}

