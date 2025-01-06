import { BindAdminRequest } from "@/canister/declarations/patient_registry/patient_registry.did";
import { adminForm } from "@/constants/contents/admin/form";
import { usePatientMethod } from "@/services/patients";
import { useToast } from "@chakra-ui/react";
import { Principal } from "@dfinity/principal";

const useAdmin = () => {
  const { success } = adminForm;
  const toast = useToast();

  const { call: bindAdmin, loading: bindAdminLoading } = usePatientMethod({
    functionName: "bind_admin",
    refetchOnMount: false,
    onSuccess() {
      toast({
        title: success.title,
        description: success.description,
        status: "success",
      });

      return;
    },
    onError(err) {
      throw err;
    },
  });

  const handleBindAdmin = async (values: {
    nik: string;
    principal: string;
  }, onClose: () => void) => {
    try {
      const principal = Principal.fromText(values.principal);

      const data: BindAdminRequest = {
        nik: values.nik,
        principal: principal,
      };

      await bindAdmin([data]);

      onClose()
    } catch (error: unknown) {
      if (error instanceof Error) {
        toast({
          title: error.name,
          description: error.message,
          status: "error",
        });
      }

      console.error(error)
    }
  };
  
  return {
    bindAdminLoading,
    handleBindAdmin,
  }
}

export default useAdmin