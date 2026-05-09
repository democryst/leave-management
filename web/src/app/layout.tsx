import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import QueryProvider from "@/lib/providers/QueryProvider";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Leave Management | Premium Dashboard",
  description: "Secure, Agentic Leave Management System",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${inter.variable} antialiased`}>
        <QueryProvider>
          <div className="min-h-screen relative overflow-hidden">
            <main className="relative z-10 p-6 md:p-12 max-w-7xl mx-auto">
              {children}
            </main>
          </div>
        </QueryProvider>
      </body>
    </html>
  );
}
