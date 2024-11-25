import { uamDeniedModal } from "@/constants/contents/uam/detail";
import { updateUam } from "@/libs/api/uam";
import { Button, Flex, FormControl, FormLabel, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, ModalOverlay, Radio, RadioGroup, Textarea, useToast } from "@chakra-ui/react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useSearchParams } from "next/navigation";
import { useState } from "react";

interface UAMDeniedModal {
  isOpen: boolean;
  onClose: () => void;
}

export default function UAMDeniedModal({
  isOpen,
  onClose
}: UAMDeniedModal) {
  const toast = useToast();
  const params = useSearchParams();

  const user = params.get('user');
  const page = params.get('page') || "1";
  const limit = params.get('limit') || "10";

  const { title, reasons, others } = uamDeniedModal;

  const [reason, setReason] = useState<string>('');
  const [otherReason, setOtherReason] = useState<string>('');

  const queryClient = useQueryClient();

  const {
    mutate: updateUser,
    isPending
  } = useMutation({
    mutationKey: ['uam', 'denied', { user }],
    mutationFn: () => updateUam({
      user: user as string,
      message: reason || otherReason,
      verification: 'rejected'
    }),
    onSuccess() {
      queryClient.invalidateQueries({
        queryKey: ['uam', { user }]
      });

      queryClient.invalidateQueries({
        queryKey: ['uam', { page, limit }]
      });

      handleClose();

      return toast({
        title: 'User Denied',
        description: 'User has been denied.',
        status: 'success',
      })
    },
    onError(error) {
      console.error(error);

      return toast({
        title: 'Error',
        description: 'Failed to deny user.',
        status: 'error',
      })
    }
  });


  const handleClose = () => {
    onClose();
    setReason('');
    setOtherReason('');
  }

  const onChangeOtherReason = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const { value } = e.target;
    setOtherReason(value);
    setReason('');
  }

  const onChangeReason = (value: string) => {
    setReason(value);
    setOtherReason('');
  }

  return (
    <Modal
      isOpen={isOpen}
      onClose={handleClose}
      closeOnOverlayClick={false}
    >
      <ModalOverlay />
      <ModalContent rounded={"xl"}>
        <ModalHeader>
          {title}
        </ModalHeader>
        <ModalBody>
          <Flex direction={'column'} gap={7}>
            <RadioGroup onChange={onChangeReason} value={reason}>
              <Flex direction={'column'} gap={3}>
                {reasons.map((reason) => (
                  <Radio key={reason.value} value={reason.value}>
                    {reason.label}
                  </Radio>
                ))}
              </Flex>
            </RadioGroup>
            <FormControl>
              <FormLabel>{others.label}</FormLabel>
              <Textarea
                placeholder={others.placeholder}
                h={32}
                maxH={32}
                onChange={onChangeOtherReason}
                value={otherReason}
              />
            </FormControl>
          </Flex>
        </ModalBody>
        <ModalFooter gap={3}>
          <Button
            colorScheme="red"
            variant={'ghost'}
            w={'full'}
            onClick={handleClose}
            isDisabled={isPending}
          >
            Cancel
          </Button>
          <Button
            colorScheme="primary"
            bg={"primary.700"}
            w={'full'}
            onClick={() => updateUser()}
            isLoading={isPending}
            isDisabled={!reason && !otherReason}
          >
            Submit
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}