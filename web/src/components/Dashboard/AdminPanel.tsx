'use client';

import { useMutation } from '@tanstack/react-query';
import { api } from '@/lib/api';
import { useState } from 'react';

export function AdminPanel() {
  const [staffId, setStaffId] = useState('');
  const [fullName, setFullName] = useState('');

  const registerMutation = useMutation({
    mutationFn: (data: any) => api.post('/api/v1/admin/staff/register', data),
  });

  const terminateMutation = useMutation({
    mutationFn: (id: string) => api.post(`/api/v1/admin/staff/terminate/${id}`, {}),
  });

  return (
    <div className="p-6 bg-white/5 border border-white/10 rounded-2xl space-y-6">
      <h2 className="text-xl font-bold text-white/90">Admin Control Center</h2>
      
      <div className="space-y-4">
        <h3 className="text-sm font-medium text-white/60">Register New Staff</h3>
        <div className="grid gap-3">
          <input 
            type="text" 
            placeholder="Staff ID"
            value={staffId}
            onChange={(e) => setStaffId(e.target.value)}
            className="w-full bg-black/20 border border-white/10 rounded-xl px-4 py-2 text-white outline-none focus:border-purple-500/50 transition-all"
          />
          <input 
            type="text" 
            placeholder="Full Name"
            value={fullName}
            onChange={(e) => setFullName(e.target.value)}
            className="w-full bg-black/20 border border-white/10 rounded-xl px-4 py-2 text-white outline-none focus:border-purple-500/50 transition-all"
          />
          <button 
            onClick={() => registerMutation.mutate({ staff_id: staffId, full_name: fullName, role: 'staff' })}
            className="w-full py-2 bg-purple-600/80 hover:bg-purple-600 text-white rounded-xl font-medium transition-all"
          >
            Register Employee
          </button>
        </div>
      </div>

      <div className="pt-6 border-t border-white/5">
        <h3 className="text-sm font-medium text-white/60">Termination</h3>
        <div className="mt-3 flex gap-2">
          <input 
            type="text" 
            placeholder="UUID to terminate"
            className="flex-1 bg-black/20 border border-white/10 rounded-xl px-4 py-2 text-white text-xs"
            id="terminate-id"
          />
          <button 
            onClick={() => {
              const id = (document.getElementById('terminate-id') as HTMLInputElement).value;
              if (id) terminateMutation.mutate(id);
            }}
            className="px-4 py-2 bg-red-500/20 text-red-400 border border-red-500/30 rounded-xl text-xs hover:bg-red-500/30 transition-all"
          >
            Terminate
          </button>
        </div>
      </div>
    </div>
  );
}
