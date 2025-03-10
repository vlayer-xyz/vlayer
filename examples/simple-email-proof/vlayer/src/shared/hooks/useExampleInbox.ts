import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { useLocalStorage } from "usehooks-ts";

const emailServiceUrl = import.meta.env.VITE_EMAIL_SERVICE_URL;

const useExampleInbox = (emailId: string | undefined) => {
  const [emlFetched, setEmlFetched] = useState(false);
  const [, setEmlFile] = useLocalStorage("emlFile", "");

  console.log("emailServiceUrl", emailServiceUrl);

  const { data, status } = useQuery({
    queryKey: ["receivedEmailEmlContent", emailId],
    queryFn: async () => {
      const response = await fetch(`${emailServiceUrl}/${emailId}.eml`);
      if (!response.ok) {
        throw new Error("Failed to fetch email");
      }
      return response.text();
    },
    enabled: !!emailId,
    retry: 6,
    retryDelay: 10000, // 10 sec delay between fetch retries
  });

  useEffect(() => {
    if (data && status === "success") {
      setEmlFile(data);
      setEmlFetched(true);
    }
  }, [data, status]);

  return { emlFetched };
};

export default useExampleInbox;
