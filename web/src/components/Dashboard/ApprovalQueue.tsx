'use client';

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '@/lib/api';

export function ApprovalQueue() {
  const queryClient = useQueryClient();

  const { data: approvals, isLoading } = useQuery({
    queryKey: ['pending-approvals'],
    queryFn: () => api.get<any[]>('/api/v1/leave/approvals'),
  });

  const approveMutation = useMutation({
    mutationFn: (id: string) => api.post(`/api/v1/leave/requests/${id}/approve`, {}),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['pending-approvals'] });
    },
  });

  if (isLoading) return <div className="animate-pulse h-20 bg-white/5 rounded-xl" />;
  if (!approvals || approvals.length === 0) return null;

  return (
    <div className="space-y-4">
      <h3 className="text-lg font-semibold text-white/90">Pending Approvals</h3>
      <div className="grid gap-3">
        {approvals.map((req: any) => (
          <div key={req.id} className="p-4 bg-white/5 border border-white/10 rounded-xl flex items-center justify-between">
            <div>
              <p className="text-sm text-white/70">Request from ID: {req.staff_id}</p>
              <p className="text-xs text-white/40">{req.start_date} to {req.end_date}</p>
            </div>
            <div className="flex gap-2">
              <button 
                onClick={() => approveMutation.mutate(req.id)}
                className="px-3 py-1 bg-green-500/20 text-green-400 border border-green-500/30 rounded-lg text-xs hover:bg-green-500/30 transition-all"
              >
                Approve
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
