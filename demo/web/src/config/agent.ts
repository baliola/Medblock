// agent.ts
import { createAgentContext } from '@ic-reactor/react';
import { CreateAgentCotextParameters } from '@ic-reactor/react/dist/types';
import { host } from './config';
import { useCentralStore } from '@/Store';

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
