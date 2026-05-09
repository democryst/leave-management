/**
 * Standardized API client for the Leave Management System.
 * Automatically injects the Edge JWT and handles error responses.
 */

export const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export const apiFetch = async <T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> => {
  // Retrieve the Edge JWT from cookies
  const cookies = document.cookie.split("; ");
  const jwt = cookies.find(row => row.startsWith("edge_jwt="))?.split("=")[1];

  const headers: HeadersInit = {
    "Content-Type": "application/json",
    ...options.headers,
  };

  if (jwt) {
    (headers as Record<string, string>)["Authorization"] = `Bearer ${jwt}`;
  }

  const response = await fetch(`${API_BASE_URL}${endpoint}`, {
    ...options,
    headers,
  });

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new Error(errorData.message || `API error: ${response.status}`);
  }

  return response.json();
};
