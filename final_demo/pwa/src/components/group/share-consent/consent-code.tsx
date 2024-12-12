import { useEffect, useState } from "react";
import { Button, Drawer, DrawerBody, DrawerContent, DrawerOverlay, Flex, Image, Skeleton, Text, useToast } from "@chakra-ui/react";
import { ClaimConsentRequest } from "@/declarations/patient_registry/patient_registry.did";

interface ConcentCodeView {
  isOpen: boolean;
  onClose: () => void;
  code: string;
  refetchCode: () => void;
  isFetching: boolean;
}

const CODE_EXPIRED_TIME = 30; // 30 SECONDS
const CHECK_INTERVAL = 10; // 10 SECONDS

export default function ConcentCodeView({
  isOpen,
  onClose,
  code,
  refetchCode,
  isFetching
}: ConcentCodeView) {
  const toast = useToast();
  const [timeRemaining, setTimeRemaining] = useState(CODE_EXPIRED_TIME);
  const [displayedCode, setDisplayedCode] = useState<string | null>(null);

  useEffect(() => {
    let timerInterval: NodeJS.Timeout | null = null;

    if (isOpen && code) {
      setDisplayedCode(code);
      setTimeRemaining(CODE_EXPIRED_TIME);

      timerInterval = setInterval(() => {
        setTimeRemaining((prev) => {
          if (prev <= 1) {
            clearInterval(timerInterval!);
            setDisplayedCode(null); // Code expired
            return 0;
          }
          return prev - 1;
        });
      }, 1000);
    }

    return () => {
      if (timerInterval) clearInterval(timerInterval);
      setDisplayedCode(null);
      setTimeRemaining(CODE_EXPIRED_TIME);
    };

    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isOpen, code]);

  return (
    <Drawer isOpen={isOpen} onClose={onClose} size={'full'} placement="bottom">
      <DrawerOverlay>
        <DrawerContent bg={"primary.700"}>
          <DrawerBody as={Flex}
            flexDirection={'column'}
            align={'center'}
            pos={'relative'}
            maxH={'100dvh'}
            overflow={'hidden'}
            px={0}
            maxW={'2xl'}
            mx={'auto'}
            p={5} py={10}
            textAlign={'center'}
            gap={5}
          >
            <Text fontSize={'2xl'} fontWeight={'bold'} color={'white'}>
              Share this Consent Code to Leader of the Group
            </Text>

            {isFetching ? (
              <Skeleton w={{ base: 'full', md: 'sm' }} rounded={'lg'} h={16} />
            ) : (
              <Text
                fontSize={displayedCode ? '6xl' : '4xl'}
                fontWeight={'bold'}
                color={'white'}
              >
                {displayedCode ?? "Code Expired"}
              </Text>
            )}

            {displayedCode ? (
              <Text color="white">
                Code expires in {timeRemaining} seconds
              </Text>
            ) : (
              <Button
                variant={'ghost'}
                textDecoration={'underline'}
                color={'warning.500'}
                fontSize={'sm'}
                onClick={refetchCode}
                isDisabled={timeRemaining > 0}
                isLoading={isFetching}
              >
                Get New Code
              </Button>
            )}

            <Button
              px={14} py={6}
              fontSize={'sm'}
              bg={"white"}
              onClick={onClose}
            >
              Back
            </Button>

            <Flex
              w={"full"}
              justify={'center'}
              align={'center'}
              h={"50dvh"}
              pos={'absolute'}
              bottom={-32}
              left={0}
              zIndex={-1}
            >
              <Image
                src={"/assets/concent-code-bg.png"}
                alt="illustration"
                objectFit={'contain'}
              />
            </Flex>
          </DrawerBody>
        </DrawerContent>
      </DrawerOverlay>
    </Drawer>
  )
}