import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { apiFetch } from "@/lib/api";

export interface StaffProfile {
  id: string;
  name: string;
  email: string;
  role: string;
}

export interface LeaveRequest {
  id: string;
  start_date: string;
  end_date: string;
  status: string;
  reason: string;
}

export interface LeaveType {
  id: string;
  name: string;
  allowance_per_year: string;
}

/**
 * Hook to fetch the current staff member's profile.
 */
export const useStaff = () => {
  return useQuery({
    queryKey: ["staff", "profile"],
    queryFn: () => apiFetch<StaffProfile>("/api/v1/staff/profile"),
  });
};

/**
 * Hook to fetch the user's leave requests history.
 */
export const useLeaveRequests = () => {
  return useQuery({
    queryKey: ["leave", "requests"],
    queryFn: () => apiFetch<LeaveRequest[]>("/api/v1/leave/requests"),
  });
};

/**
 * Hook to fetch available leave types from the Policy service.
 */
export const useLeaveTypes = () => {
  return useQuery({
    queryKey: ["policy", "leave-types"],
    queryFn: () => apiFetch<LeaveType[]>("/api/v1/policy/leave-types"),
  });
};

/**
 * Hook to submit a new leave request.
 */
export const useCreateLeaveRequest = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (data: Partial<LeaveRequest>) =>
      apiFetch("/api/v1/leave/requests", {
        method: "POST",
        body: JSON.stringify(data),
      }),
    onSuccess: () => {
      // Invalidate and refetch leave requests list
      queryClient.invalidateQueries({ queryKey: ["leave", "requests"] });
    },
  });
};
