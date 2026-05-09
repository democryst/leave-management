import React, { useState } from 'react';
import { Send, Calendar } from 'lucide-react';
import { useLeaveTypes, useCreateLeaveRequest } from '@/hooks/useDashboard';

export const LeaveForm: React.FC = () => {
  const { data: leaveTypes } = useLeaveTypes();
  const createRequest = useCreateLeaveRequest();
  
  const [formData, setFormData] = useState({
    start_date: '',
    end_date: '',
    leave_type_id: '',
    reason: '',
  });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    createRequest.mutate(formData);
  };

  return (
    <div className="glass-card p-6">
      <div className="flex items-center gap-2 mb-6">
        <Calendar className="w-5 h-5 text-blue-400" />
        <h2 className="text-xl font-bold text-white">Apply for Leave</h2>
      </div>
      
      <form onSubmit={handleSubmit} className="space-y-4">
        <div className="grid grid-cols-2 gap-4">
          <div className="space-y-1">
            <label className="text-xs text-slate-400 uppercase tracking-widest">Start Date</label>
            <input 
              type="date" 
              className="w-full bg-slate-900/50 border border-white/10 rounded-lg p-2 text-white focus:outline-none focus:border-blue-500"
              value={formData.start_date}
              onChange={(e) => setFormData({...formData, start_date: e.target.value})}
            />
          </div>
          <div className="space-y-1">
            <label className="text-xs text-slate-400 uppercase tracking-widest">End Date</label>
            <input 
              type="date" 
              className="w-full bg-slate-900/50 border border-white/10 rounded-lg p-2 text-white focus:outline-none focus:border-blue-500"
              value={formData.end_date}
              onChange={(e) => setFormData({...formData, end_date: e.target.value})}
            />
          </div>
        </div>

        <div className="space-y-1">
          <label className="text-xs text-slate-400 uppercase tracking-widest">Leave Type</label>
          <select 
            className="w-full bg-slate-900/50 border border-white/10 rounded-lg p-2 text-white focus:outline-none focus:border-blue-500"
            value={formData.leave_type_id}
            onChange={(e) => setFormData({...formData, leave_type_id: e.target.value})}
          >
            <option value="">Select a type...</option>
            {leaveTypes?.map(type => (
              <option key={type.id} value={type.id}>{type.name}</option>
            ))}
          </select>
        </div>

        <div className="space-y-1">
          <label className="text-xs text-slate-400 uppercase tracking-widest">Reason</label>
          <textarea 
            className="w-full bg-slate-900/50 border border-white/10 rounded-lg p-2 text-white focus:outline-none focus:border-blue-500"
            rows={3}
            value={formData.reason}
            onChange={(e) => setFormData({...formData, reason: e.target.value})}
          />
        </div>

        <button 
          type="submit" 
          disabled={createRequest.isPending}
          className="btn-primary w-full flex items-center justify-center gap-2"
        >
          {createRequest.isPending ? "Submitting..." : (
            <>
              <Send className="w-4 h-4" />
              <span>Submit Request</span>
            </>
          )}
        </button>
      </form>
    </div>
  );
};
