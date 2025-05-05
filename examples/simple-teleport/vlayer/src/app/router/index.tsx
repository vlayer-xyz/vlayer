import { BrowserRouter, Routes, Route } from "react-router";
import { ErrorBoundary } from "react-error-boundary";
import { AppErrorBoundaryComponent } from "../../shared/errors/ErrorBoundary";
import { Layout } from "../../shared/layout/Layout";
import { getAllSteps } from "./steps";

const Router = () => {
  return (
    <BrowserRouter>
      <ErrorBoundary FallbackComponent={AppErrorBoundaryComponent}>
        <Routes>
          <Route path="/" element={<Layout />}>
            {getAllSteps().map((step) => (
              <Route
                key={step.path}
                path={step.path}
                element={<step.component />}
              />
            ))}
          </Route>
        </Routes>
      </ErrorBoundary>
    </BrowserRouter>
  );
};

export default Router;
