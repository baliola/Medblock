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

export function createCanisterError(error: unknown): CanisterError | null {
  const errorObj = error as Error;
  const errorMessage = errorObj.message;

  // Regular expression pattern to match the error format
  const match = errorMessage.match(/Reject code: (\d+)\s+Reject text: (.+)/);

  if (match) {
    const [, rejectCode, rejectText] = match;
    // Create CanisterError with reject code and text
    return new CanisterError(
      errorMessage,
      '', // Empty placeholders for Canister, Method, Status, and Code as they are not available
      '',
      '',
      '',
      `Reject code: ${rejectCode}, Reject text: ${rejectText}`,
    );
  }

  // Update the existing code to handle the Canister error format
  // (assuming it's the same as before)
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

  return null;
}
