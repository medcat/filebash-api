name := "com.filebash.api"

version := "0.1"

scalaVersion := "2.12.7"

libraryDependencies ++= Seq(
  "com.typesafe" % "config" % "1.3.2",
  "com.typesafe.akka" %% "akka-actor" % "2.5.18",
  "com.typesafe.akka" %% "akka-stream" % "2.5.18",
  "com.typesafe.akka" %% "akka-http" % "10.1.5",
  "com.typesafe.akka" %% "akka-http-spray-json" % "10.1.5",
  "ch.qos.logback" % "logback-classic" % "1.2.3",
  "com.typesafe.scala-logging" %% "scala-logging" % "3.9.0",
  "io.spray" %% "spray-json" % "1.3.5",
  "io.rdbc" %% "rdbc-api-scala" % "0.0.82",
  "io.rdbc" %% "rdbc-api-scala" % "0.0.82",
  "io.rdbc.pool" %% "rdbc-pool-scala" % "0.0.11",
  "io.rdbc" %% "rdbc-api-scala" % "0.0.82",
  "io.rdbc.pgsql" %% "pgsql-transport-netty" % "0.4.0.1",
  "com.typesafe.akka" %% "akka-testkit" % "2.5.18" % Test,
  "com.typesafe.akka" %% "akka-stream-testkit" % "2.5.18" % Test,
  "com.typesafe.akka" %% "akka-http-testkit" % "10.1.5" % Test
)
