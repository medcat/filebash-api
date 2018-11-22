package com.filebash.api

import java.util.concurrent.atomic.AtomicLong
import java.util.concurrent.{Executors, ThreadFactory}

import scala.concurrent.ExecutionContext

object NiceExecutionContext {
  def apply(name: String): ExecutionContext =
    ExecutionContext.fromExecutorService(Executors.newCachedThreadPool(new NiceThreadFactory(name)))

  private class NiceThreadFactory(prefix: String) extends ThreadFactory {
    private val threadGroup = Thread.currentThread().getThreadGroup
    private val threadNumber = new AtomicLong(1)

    override def newThread(r: Runnable): Thread =
      new Thread(threadGroup, r, s"$prefix/$threadNumber", 0)
  }

}
