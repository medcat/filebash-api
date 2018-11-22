package com.filebash.api.http.routes

import akka.http.scaladsl.server.Directives
import akka.http.scaladsl.server.directives.Credentials
import com.filebash.api.Store
import com.filebash.api.model.User

import scala.concurrent.{ExecutionContext, Future}

object SessionRoute extends Directives {
  def create(store: Store)(implicit ec: ExecutionContext) =
    authenticateBasicAsync("filebash", authenticator(store)) {
    ???
  }

  private def authenticator(store: Store)(credentials: Credentials)
                           (implicit ec: ExecutionContext): Future[Option[User]] =
    credentials match {
      case Credentials.Missing => Future.successful(None),
      case Credentials.Provided(ident) => {
        User.findUser(ident)(store).map { userOpt =>
          userOpt.filter { user =>
            credentials.verify(user.pass, User.verifyHash(user.pass))
          }
        }
      }
    }
}
