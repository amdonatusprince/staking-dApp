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
  const date = new Date();

  const year = date.getUTCFullYear();
  const month = String(date.getUTCMonth() + 1).padStart(2, "0"); // Months are 0-based
  const day = String(date.getUTCDate()).padStart(2, "0");
  const hours = String(date.getUTCHours()).padStart(2, "0");
  const minutes = String(date.getUTCMinutes()).padStart(2, "0");
  const seconds = String(date.getUTCSeconds()).padStart(2, "0");
  const milliseconds = String(date.getUTCMilliseconds()).padStart(3, "0");

  // Combine the parts into the desired format
  const formattedDate = `${year}-${month}-${day}T${hours}:${minutes}:${seconds}.${milliseconds}+00:00`;

  return formattedDate;
}

export function compareTimestamps(currentTime: string, unlockTime: string) {
  const datetime1 = new Date(currentTime);
  const datetime2 = new Date(unlockTime);

  // console.log(datetime1 >= datetime2);
  return datetime1 >= datetime2;
}
