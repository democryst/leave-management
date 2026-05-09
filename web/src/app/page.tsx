"use client";

import { ProfileCard } from "@/components/Dashboard/ProfileCard";
import { LeaveForm } from "@/components/Dashboard/LeaveForm";
import { HistoryTable } from "@/components/Dashboard/HistoryTable";
import { useStaff, useLeaveRequests } from "@/hooks/useDashboard";

export default function Home() {
  const { data: staff, isLoading: staffLoading } = useStaff();
  const { data: requests, isLoading: requestsLoading } = useLeaveRequests();

  return (
    <div className="space-y-12">
      <header className="flex flex-col gap-2">
        <h1 className="text-4xl font-extrabold text-white tracking-tight">
          Staff Dashboard
        </h1>
        <p className="text-slate-400">
          Manage your leave requests and profile information in one place.
        </p>
      </header>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 items-start">
        {/* Left Column: Profile & Form */}
        <div className="lg:col-span-1 space-y-8">
          <ProfileCard staff={staff} isLoading={staffLoading} />
          <LeaveForm />
        </div>

        {/* Right Column: History */}
        <div className="lg:col-span-2">
          <HistoryTable requests={requests} isLoading={requestsLoading} />
        </div>
      </div>
    </div>
  );
}
