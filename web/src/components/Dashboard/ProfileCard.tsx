import React from 'react';
import { User, Mail, Shield } from 'lucide-react';
import { StaffProfile } from '@/hooks/useDashboard';

export const ProfileCard: React.FC<{ staff?: StaffProfile; isLoading: boolean }> = ({ staff, isLoading }) => {
  if (isLoading) return <div className="glass-card p-6 animate-pulse h-48" />;
  
  return (
    <div className="glass-card p-6 flex flex-col gap-4">
      <div className="flex justify-between items-center border-b border-white/10 pb-4">
        <h2 className="text-xl font-bold text-white">Staff Profile</h2>
        <div className="bg-purple-500/20 p-2 rounded-full">
           <User className="w-5 h-5 text-purple-400" />
        </div>
      </div>
      
      <div className="space-y-3">
        <div className="flex items-center gap-3">
          <Mail className="w-4 h-4 text-blue-400" />
          <span className="text-slate-300">{staff?.email}</span>
        </div>
        <div className="flex items-center gap-3">
          <Shield className="w-4 h-4 text-purple-400" />
          <span className="text-slate-300 font-medium uppercase text-xs tracking-wider">{staff?.role}</span>
        </div>
      </div>

      <div className="mt-4 pt-4 border-t border-white/5">
         <p className="text-2xl font-bold text-white">{staff?.name}</p>
         <p className="text-slate-500 text-sm">Employee ID: {staff?.id.slice(0, 8)}</p>
      </div>
    </div>
  );
};
