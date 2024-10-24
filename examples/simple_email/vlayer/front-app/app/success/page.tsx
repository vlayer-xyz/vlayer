"use client";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { CircleCheckBigIcon } from "lucide-react";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";

export default function Success() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const txHash = searchParams.get("txHash");

  return (
    <div className="flex justify-center items-center min-h-screen p-4 bg-gray-950">
      <Card className="w-full max-w-md border-violet-500 bg-gray-900 text-white">
        <CardHeader>
          <CardTitle className="text-center">
            Success, your NFT is ready!
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex justify-center mt-10 mb-10">
            <CircleCheckBigIcon className="w-24 h-24 text-green-400" />
          </div>

          <div className="flex space-x-4 mt-4 justify-center">
            <Link
              href={`https://sepolia-optimism.etherscan.io/tx/${txHash}`}
              className="px-4 py-2 bg-gray-600 text-white rounded"
            >
              Block explorer
            </Link>
            <button
              onClick={() => router.back()}
              className="px-4 py-2 bg-gray-800 text-white rounded"
            >
              Back
            </button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
