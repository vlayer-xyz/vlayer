import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "vlayer Email Proof example",
  description: "Generate proof of specific email domain.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
