import Actions from "./containers/Actions";
import ErrorBoundary from "./components/ErrorBoundry";

function App() {
  return (
    <ErrorBoundary>
      <Actions />
    </ErrorBoundary>
  );
}

export default App;
