import { BrowserRouter, Routes, Route } from "react-router";
import { Layout } from "../../shared/layout/Layout";
import { steps } from "./steps";

const Router = () => {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          {steps.map((step) => (
            <Route
              key={step.path}
              path={step.path}
              element={<step.component />}
            />
          ))}
        </Route>
      </Routes>
    </BrowserRouter>
  );
};

export default Router;
