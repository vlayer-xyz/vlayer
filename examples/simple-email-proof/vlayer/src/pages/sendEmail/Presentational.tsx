import { InputWithCopy } from "./ui/InputWithCopy";
import { NextButton } from "./ui/NextButton";

export const SendEmail = ({
  subject,
  uniqueEmail,
}: {
  subject: string;
  uniqueEmail: string;
}) => {
  return (
    <>
      <div className="w-full">
        <InputWithCopy label="To" value={uniqueEmail} />
        <InputWithCopy label="Subject" value={subject} />
      </div>
      <div className="mt-5 flex justify-center">
        <NextButton path={`/collect-email?uniqueEmail=${uniqueEmail}`} />
      </div>
    </>
  );
};
