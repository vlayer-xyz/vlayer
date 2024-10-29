import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Email Proof for domain",
  description: "Mint NFT for specifc domain email owner",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`antialiased`}>{children}</body>
    </html>
  );
}
