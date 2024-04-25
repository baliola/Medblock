import { ErrorMessages } from './constant';

class CanisterError extends Error {
  canister: string;
  method: string;
  status: string;
  code: string;
  errorType?: string; // Make errorType optional
  additionalInfo?: string; // Add additionalInfo property for extra error details

  constructor(
    message: string,
    canister: string,
    method: string,
    status: string,
    code: string,
    errorType?: string,
    additionalInfo?: string,
  ) {
    super(message);
    this.name = 'CanisterError';
    this.canister = canister;
    this.method = method;
    this.status = status;
    this.code = code;
    this.errorType = errorType;
    this.additionalInfo = additionalInfo; // Assign additionalInfo to the property
  }
}

class HttpAgentError extends Error {
  requestId: string;
  rejectCode: number;
  rejectText: string;

  constructor(
    message: string,
    requestId: string,
    rejectCode: number,
    rejectText: string,
  ) {
    super(message);
    this.name = 'HttpAgentError';
    this.requestId = requestId;
    this.rejectCode = rejectCode;
    this.rejectText = rejectText;
  }
}

export function createCanisterError(
  error: unknown,
): CanisterError | HttpAgentError | null {
  const errorObj = error as Error;
  const errorMessage = errorObj.message;

  // Regular expression pattern to match the error format
  const canisterMatch = errorMessage.match(
    /Canister: (.+)\s+Method: (.+)\s+"Status": "(.+)"\s+"Code": "(.+)"\s+"Message": "(.+)"/,
  );

  if (canisterMatch) {
    const [, canister, method, status, code, message] = canisterMatch;
    if (message.includes(ErrorMessages.ProviderDoesNotExist)) {
      // Create CanisterError for ProviderDoesNotExist
      return new CanisterError(
        message,
        canister,
        method,
        status,
        code,
        ErrorMessages.ProviderDoesNotExist,
      );
    } else {
      // Create CanisterError without reject code and text
      return new CanisterError(message, canister, method, status, code);
    }
  }

  // Regular expression pattern to match HTTP agent error format
  const httpAgentMatch = errorMessage.match(
    /AgentError ID: (.+)\s+Code: (\d+)\s+Body: (.+)/,
  );

  if (httpAgentMatch) {
    const [, requestId, rejectCodeStr, rejectText] = httpAgentMatch;
    const rejectCode = parseInt(rejectCodeStr, 10);
    // Create HttpAgentError
    return new HttpAgentError(errorMessage, requestId, rejectCode, rejectText);
  }

  return null;
}
