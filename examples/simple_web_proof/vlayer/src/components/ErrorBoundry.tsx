import React, { ErrorInfo } from "react";

interface ErrorState {
  hasError: boolean;
  hint: string;
}

class ErrorBoundary extends React.Component<
  { children: React.ReactNode },
  ErrorState
> {
  constructor(props: { children: React.ReactNode }) {
    super(props);
    this.state = { hasError: false, hint: "" };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error({ error, errorInfo });

    if (error.stack?.includes("connectToExtension")) {
      this.setState({
        hasError: true,
        hint: "Have you installed <b><a href='https://chromewebstore.google.com/detail/vlayer/jbchhcgphfokabmfacnkafoeeeppjmpl' target='_blank'>vlayer extension</a></b>?",
      });
    } else {
      this.setState({ hasError: true, hint: "Check dev console for details" });
    }
  }

  render() {
    if (this.state.hasError) {
      return (
        <div className="min-h-screen flex flex-col items-center justify-center p-4">
          <div className="card w-96 shadow-xl bg-violet-100">
            <div className="card-body items-center text-center space-y-4">
              <p className="text-slate-600 text-center">
                Something went wrong <br />
                <i dangerouslySetInnerHTML={{ __html: this.state.hint }} />
              </p>
            </div>
          </div>
        </div>
      );
    }

    return this.props.children;
  }
}

export default ErrorBoundary;
