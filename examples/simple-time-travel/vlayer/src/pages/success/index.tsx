import { useSearchParams } from "react-router";

export const SuccessPage = () => {
  const [searchParams] = useSearchParams();
  const txHash = searchParams.get("txHash");

  return (
    <div className="mt-5 flex justify-center text-slate-900">
      <div>Success: {txHash}</div>
    </div>
  );
};
