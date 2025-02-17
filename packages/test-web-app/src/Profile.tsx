import React, { useEffect, useState } from "react";

function Profile() {
  return ProfileWithUrl(
    "https://lotr-api.online:3011/regular_json?are_you_sure=yes&really=yes&auth=s3cret_t0ken",
  );
}

function ProfileFailedAuth() {
  return ProfileWithUrl(
    "https://lotr-api.online:3011/regular_json?are_you_sure=yes&really=yes",
  );
}

function ProfileWithUrl(fetched_url: string) {
  const [gandalf, setGandalf] = useState<{ name: string } | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  useEffect(() => {
    const getGandalf = async () => {
      setIsLoading(true);
      const response = await fetch(fetched_url);
      const data = (await response.json()) as { name: string };
      setGandalf(data);
      setIsLoading(false);
    };
    getGandalf().catch(console.error);
  }, []);
  return (
    <div className="container">
      <h1> Profile page of </h1>
      {isLoading ? <p>Loading...</p> : <p>{gandalf?.name}</p>}
    </div>
  );
}

export { Profile, ProfileFailedAuth };
