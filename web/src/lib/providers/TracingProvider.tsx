"use client";

import { useEffect } from "react";
import { initTracing } from "@/lib/tracing";

export default function TracingProvider({ children }: { children: React.ReactNode }) {
  useEffect(() => {
    initTracing();
  }, []);

  return <>{children}</>;
}
