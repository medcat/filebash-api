package com.filebash.api.model

import com.filebash.api.Store
import io.rdbc.sapi.{Row, Timeout}

import scala.concurrent.Future

object User {
  def findUser(email: String)(store: Store): Future[Option[User]] = {
    store.withConnection(conn =>
      conn.statement("SELECT * FROM users WHERE email = :email LIMIT 1")
        .bind("email" -> email)
        .executeForValue(User.fromRow)(Timeout.Inf))
  }

  def fromRow(row: Row): User =
    User(row.col("email"), row.col("password"))

  def verifyHash(stored: String)(given: String): String = ???
}

final case class User(email: String, pass: String)
