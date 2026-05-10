"use client";

import { ProfileCard } from "@/components/Dashboard/ProfileCard";
import { LeaveForm } from "@/components/Dashboard/LeaveForm";
import { HistoryTable } from "@/components/Dashboard/HistoryTable";
import { ApprovalQueue } from "@/components/Dashboard/ApprovalQueue";
import { AdminPanel } from "@/components/Dashboard/AdminPanel";
import { useStaff, useLeaveRequests } from "@/hooks/useDashboard";

export default function Home() {
  const { data: staff, isLoading: staffLoading } = useStaff();
  const { data: requests, isLoading: requestsLoading } = useLeaveRequests();

  const isAdmin = staff?.role === 'admin';
  const isManager = staff?.role === 'manager' || isAdmin;

  return (
    <div className="space-y-12 pb-20">
      <header className="flex flex-col gap-2">
        <h1 className="text-4xl font-extrabold text-white tracking-tight">
          Sovereign Leave Management
        </h1>
        <p className="text-slate-400">
          Secure, local-first HR orchestration for autonomous systems.
        </p>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 items-start">
        {/* Left Column: Profile & Form */}
        <div className="lg:col-span-1 space-y-8">
          <ProfileCard staff={staff} isLoading={staffLoading} />
          {isManager && <ApprovalQueue />}
          <LeaveForm />
          {isAdmin && <AdminPanel />}
        </div>

        {/* Right Column: History */}
        <div className="lg:col-span-2 space-y-8">
          <HistoryTable requests={requests} isLoading={requestsLoading} />
        </div>
      </div>
    </div>
  );
}
