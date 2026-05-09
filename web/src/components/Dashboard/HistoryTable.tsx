import React from 'react';
import { Clock, CheckCircle, XCircle, AlertCircle } from 'lucide-react';
import { LeaveRequest } from '@/hooks/useDashboard';

export const HistoryTable: React.FC<{ requests?: LeaveRequest[]; isLoading: boolean }> = ({ requests, isLoading }) => {
  if (isLoading) return <div className="glass-card p-6 animate-pulse h-64" />;

  const getStatusIcon = (status: string) => {
    switch (status.toLowerCase()) {
      case 'approved': return <CheckCircle className="w-4 h-4 text-green-400" />;
      case 'rejected': return <XCircle className="w-4 h-4 text-red-400" />;
      case 'pending': return <Clock className="w-4 h-4 text-yellow-400" />;
      default: return <AlertCircle className="w-4 h-4 text-slate-400" />;
    }
  };

  return (
    <div className="glass-card overflow-hidden">
      <div className="p-6 border-b border-white/10">
        <h2 className="text-xl font-bold text-white">Leave History</h2>
      </div>
      <div className="overflow-x-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-white/5">
              <th className="p-4 text-xs font-semibold text-slate-400 uppercase tracking-widest">Date Range</th>
              <th className="p-4 text-xs font-semibold text-slate-400 uppercase tracking-widest">Reason</th>
              <th className="p-4 text-xs font-semibold text-slate-400 uppercase tracking-widest text-center">Status</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5">
            {requests?.length === 0 && (
              <tr>
                <td colSpan={3} className="p-8 text-center text-slate-500 italic">No leave requests found.</td>
              </tr>
            )}
            {requests?.map((req) => (
              <tr key={req.id} className="hover:bg-white/5 transition-colors">
                <td className="p-4">
                  <div className="text-white font-medium">{req.start_date}</div>
                  <div className="text-slate-500 text-xs">{req.end_date}</div>
                </td>
                <td className="p-4 text-slate-300 text-sm">{req.reason}</td>
                <td className="p-4">
                  <div className="flex items-center justify-center gap-2 px-3 py-1 rounded-full bg-white/5 border border-white/10">
                    {getStatusIcon(req.status)}
                    <span className="text-xs font-semibold uppercase">{req.status}</span>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};
