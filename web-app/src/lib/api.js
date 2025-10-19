const JSON_HEADERS = {
  Accept: "application/json",
};

async function parseJsonResponse(response) {
  const text = await response.text();
  const hasBody = text !== "";
  let data;
  if (hasBody) {
    try {
      data = JSON.parse(text);
    } catch (error) {
      data = { raw: text };
    }
  }

  if (!response.ok) {
    const message = data?.error || response.statusText || "Request failed";
    const err = new Error(message);
    err.status = response.status;
    err.body = data;
    throw err;
  }

  return data ?? {};
}

export async function listFiles() {
  const response = await fetch("/api/files/list", {
    method: "GET",
    headers: JSON_HEADERS,
  });
  const data = await parseJsonResponse(response);
  return Array.isArray(data.files) ? data.files : [];
}

export async function createQuery(payload) {
  const response = await fetch("/api/query/create", {
    method: "POST",
    headers: {
      ...JSON_HEADERS,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });
  const data = await parseJsonResponse(response);
  if (!data.id) {
    throw new Error("Query creation did not return an id");
  }
  return data;
}

export async function getQueryStatus(id) {
  const response = await fetch(`/api/query/status?id=${encodeURIComponent(id)}`, {
    method: "GET",
    headers: JSON_HEADERS,
  });
  return parseJsonResponse(response);
}

export async function getQueryResult(id) {
  const response = await fetch(`/api/query/result?id=${encodeURIComponent(id)}`, {
    method: "GET",
    headers: JSON_HEADERS,
  });
  return parseJsonResponse(response);
}

export async function cancelQuery(id) {
  const response = await fetch(`/api/query/cancel?id=${encodeURIComponent(id)}`, {
    method: "GET",
    headers: JSON_HEADERS,
  });
  return parseJsonResponse(response);
}
