import { CollectEmail } from "./Presentational";
import { useSearchParams, useNavigate } from "react-router";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { useLocalStorage } from "usehooks-ts";

export const CollectEmailContainer = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const uniqueEmail = searchParams.get("uniqueEmail");

  const emailId = uniqueEmail?.split("@")[0];

  const [, setEmlFile] = useLocalStorage("emlFile", "");

  const { data, status } = useQuery({
    queryKey: ["emailVerification", uniqueEmail],
    queryFn: async () => {
      const response = await fetch(
        `https://email-example-inbox.s3.us-east-2.amazonaws.com/${emailId}.eml`,
      );
      if (!response.ok) {
        throw new Error("Failed to fetch email");
      }
      return response.text();
    },
    enabled: !!uniqueEmail,
    retry: 6,
    retryDelay: 10000, // 10 second between retries
  });

  useEffect(() => {
    if (data && status === "success") {
      setEmlFile(data);
      navigate("/mint-nft");
    }
  }, [data, status]);

  return <CollectEmail />;
};
