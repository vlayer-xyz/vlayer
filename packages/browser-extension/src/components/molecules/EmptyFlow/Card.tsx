import * as React from "react";
import { CardContent } from "./CardContent";

export const EmptyFlowCard: React.FC = () => {
  const cardData = {
    imageSrc: "/box.svg",
    title: "Nothing to prove",
    description: "No proving context has been provided ",
  };

  return (
    <div style={{ marginTop: "50%" }} data-testid="empty-flow-card">
      <CardContent {...cardData} />
    </div>
  );
};
