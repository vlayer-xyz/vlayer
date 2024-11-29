function Menu({ requestProve }: { requestProve: () => Promise<void> }) {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center p-4">
      <div className="card w-96 shadow-xl bg-violet-100">
        <figure className="px-10 pt-10">
          <img 
            src="vlayer_logo.svg" 
            alt="Logo" 
            className="rounded-xl object-cover"
          />
        </figure>
        <div className="card-body items-center text-center space-y-4">
          <div className="btn-group-vertical w-full">
            <button 
              className="btn btn-primary w-full mb-2 text-white" 
              onClick={async () => await requestProve()}
            >
              Generate Proof of Twitter
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Menu;
