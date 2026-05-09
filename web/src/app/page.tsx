export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] gap-8">
      <div className="glass-card p-12 text-center max-w-2xl animate-in fade-in slide-in-from-bottom-8 duration-700">
        <h1 className="text-5xl font-extrabold mb-6 bg-gradient-to-r from-purple-400 to-blue-400 bg-clip-text text-transparent">
          Agentic Leave System
        </h1>
        <p className="text-slate-400 text-lg mb-8 leading-relaxed">
          Experience a production-grade, microservices-based leave management workflow 
          governed by autonomous AI agents. Secure, observable, and built for scale.
        </p>
        <div className="flex gap-4 justify-center">
          <button className="btn-primary">Get Started</button>
          <button className="px-6 py-3 rounded-lg border border-slate-700 hover:bg-slate-800 transition-colors font-semibold">
            Documentation
          </button>
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 w-full max-w-5xl">
        <div className="glass-card p-6 border-t-2 border-purple-500/50">
          <h3 className="font-bold text-xl mb-2">Microservices</h3>
          <p className="text-slate-400 text-sm">Isolated domains for Staff, Leave, and Policy rules.</p>
        </div>
        <div className="glass-card p-6 border-t-2 border-blue-500/50">
          <h3 className="font-bold text-xl mb-2">Secure IST</h3>
          <p className="text-slate-400 text-sm">Internal Service Token trust chain using RS256.</p>
        </div>
        <div className="glass-card p-6 border-t-2 border-green-500/50">
          <h3 className="font-bold text-xl mb-2">Observability</h3>
          <p className="text-slate-400 text-sm">Full OTel distributed tracing with Jaeger integration.</p>
        </div>
      </div>
    </div>
  );
}
