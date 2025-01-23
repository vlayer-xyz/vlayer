import { useEffect, useRef, useState } from "react";
import { Link } from "react-router";
import { useLocalStorage } from "usehooks-ts";

interface TwitterFollower {
  screen_name: string;
  name: string;
  profile_image_url: string;
  followers_count: number;
}

export const Success = () => {
  const [loading, setLoading] = useState<boolean>(false);
  const [followers, setFollowers] = useState<TwitterFollower[]>([]);
  const [decodedTranscript] = useLocalStorage("decodedTranscript", "");
  const modalRef = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    modalRef.current?.showModal();
  }, []);

  useEffect(() => {
    setLoading(true);
    if (decodedTranscript) {
      const parsed = JSON.parse(decodedTranscript);
      const [, bodyPart] = parsed.recv.split("\r\n\r\n");
      const parsedBody = JSON.parse(bodyPart);
      const followers =
        parsedBody.data.user.result.timeline.timeline.instructions[3].entries.map(
          (entry: any) => {
            const profile =
              entry?.content?.itemContent?.user_results?.result?.legacy;
            if (profile) {
              return {
                screen_name: profile.screen_name,
                name: profile.name,
                profile_image_url: profile.profile_image_url_https,
                followers_count: profile.followers_count,
              } satisfies TwitterFollower;
            }
          })
          .filter((f) => f !== undefined);
      setFollowers(followers);
      setLoading(false);
    }
  }, [decodedTranscript]);

  return (
    <>
      <button className="btn" onClick={() => modalRef.current?.showModal()}>
        Start
      </button>
      <dialog id="my_modal_3" className="modal" ref={modalRef}>
        <div className="modal-box bg-white rounded-2xl ">
          <div className="flex justify-center">
            <img
              src="/success-illustration.svg"
              alt="Success Icon"
              className="w-[282px] h-[155px]"
            />
          </div>
          <h3 className="mt-7 header">List of Followers</h3>
          {loading && <p className="py-4 text-gray-500">Loading...</p>}
          {!loading && (
            <div className="flex flex-col gap-4 text-gray-400 text-sm">
              {followers.map((follower) => (
                <div
                  key={follower.screen_name}
                  className="flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50"
                >
                  <img
                    src={follower.profile_image_url}
                    alt={follower.name}
                    className="w-10 h-10 rounded-full"
                  />
                  <div className="flex flex-col">
                    <p className="text-sm text-gray-500">
                      @{follower.screen_name}
                    </p>
                    <p className="text-xs text-gray-400">
                      {follower.followers_count.toLocaleString()} followers
                    </p>
                  </div>
                </div>
              ))}
            </div>
          )}
          <div className="mt-7 flex justify-center">
            <Link to="/" id="nextButton">
              Start again
            </Link>
          </div>
        </div>
      </dialog>
    </>
  );
};
