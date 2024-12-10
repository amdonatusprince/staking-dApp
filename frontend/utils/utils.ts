export const formatDate = (timestamp: any) => {
  if (!timestamp) return "-";
  const milliseconds = Number(timestamp) * 1000;
  return new Date(milliseconds).toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
};

export const formatTime = (timestamp: any) => {
  if (!timestamp) return "-";
  const milliseconds = Number(timestamp) * 1000;
  return new Date(milliseconds).toLocaleTimeString("en-US", {
    hour: "2-digit",
    minute: "2-digit",
    second: "2-digit",
  });
};

export function getCurrentDateTime() {
  return Math.floor(Date.now() / 1000).toString();
}

export function compareTimestamps(currentTime: string, unlockTime: string) {
  // Convert both timestamps to milliseconds
  const currentMs = Number(currentTime) * 1000;
  const unlockMs = Number(unlockTime) * 1000;
  
  const currentDate = new Date(currentMs);
  const unlockDate = new Date(unlockMs);
  
  return currentDate >= unlockDate;
}
