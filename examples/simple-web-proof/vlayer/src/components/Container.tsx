import React from "react";

export const Container = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className="min-h-screen w-screen bg-gradient-to-br from-violet-600 to-white flex items-center justify-center p-8">
      <div className="card w-96 bg-white/90 shadow-xl p-8">
        <h2 className="card-title text-2xl font-bold text-primary mb-6 text-center flex justify-center">
          vlayer Web Proof
        </h2>
        <div className="flex flex-col gap-6">{children}</div>
      </div>
    </div>
  );
};
