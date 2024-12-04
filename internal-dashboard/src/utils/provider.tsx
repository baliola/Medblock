import { Status } from "@/canister/declarations/provider_registry/provider_registry.did";
import SuspendHospitalButton from "@/components/ham/detail/button/suspend";
import UnsuspendHospitalButton from "@/components/ham/detail/button/unsuspend";
import { ProviderStatus } from "@/constants/contents/ham/table";
import { Tag } from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";
import { ReactElement } from "react";

export interface ISuspendUnsuspendButtonProps {
  principal: Principal;
  id: string
};

export const getProviderStatus = (status: Status): ReactElement => {
  switch (Object.keys(status)[0]) {
    case ProviderStatus.Active:
      return (
        <Tag colorScheme="green" lineHeight={1}>
          {ProviderStatus.Active}
        </Tag>
      );
    case ProviderStatus.Suspended:
      return (
        <Tag colorScheme="red" lineHeight={1}>
          {ProviderStatus.Suspended}
        </Tag>
      );
    default:
      return <></>;
  }
};

export const getProviderStatusButton = (
  status: Status,
  principal: Principal,
  id: string
): ReactElement => {
  switch (Object.keys(status)[0]) {
    case ProviderStatus.Active:
      return <SuspendHospitalButton props={{ principal, id }} />;
    case ProviderStatus.Suspended:
      return <UnsuspendHospitalButton props={{ principal, id }} />;
    default:
      return <></>;
  }
};
