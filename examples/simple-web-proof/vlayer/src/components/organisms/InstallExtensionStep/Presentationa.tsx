import { vlayerPovingExtensionId } from "../../../utils";

export const InstallExtensionPresentational = () => {
  return (
    <>
      <div className="mt-7 flex justify-center">
        <button
          id="nextButton"
          onClick={() => {
            window.open(
              `https://chromewebstore.google.com/detail/vlayer/${vlayerPovingExtensionId}/reviews`,
              "_blank",
            );
          }}
        >
          Install Extension
        </button>
      </div>
    </>
  );
};
