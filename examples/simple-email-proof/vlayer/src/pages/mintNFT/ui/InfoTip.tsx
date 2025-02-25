export const InfoTip = () => {
  return (
    <div className="flex mt-5 gap-4 items-start self-stretch px-2 py-3 font-semibold rounded-lg bg-slate-50">
      <img
        loading="lazy"
        src="/img/help-icon.svg"
        alt=""
        className="object-contain shrink-0 w-10 aspect-square mt-2 ml-2"
      />
      <div className="flex flex-col min-w-[240px] w-[409px]">
        <div className="text-sm leading-5 text-black">
          Not sure how to copy the original message?
        </div>
        <div className="text-xs leading-4 text-violet-500 text-slate-600">
          To copy the email code, go to your email account, open the “Sent”
          click “Copy Raw” in the menu. Keep in mind that the copying process
          might look different on Gmail or Outlook.
        </div>
      </div>
    </div>
  );
};
