// agent.ts
import { createAgentContext } from '@ic-reactor/react';
import { CreateAgentCotextParameters } from '@ic-reactor/react/dist/types';

import { useCentralStore } from '@/Store';

import { host } from './config';

// Optional: Define custom agent configuration
const agentConfig: CreateAgentCotextParameters = {
  withLocalEnv: true,
  host,
};

export const {
  AgentProvider,
  useAgent,
  useAuth,
  useAuthState,
  useAgentState,
  useAgentManager,
  useUserPrincipal,
} = createAgentContext(agentConfig);
