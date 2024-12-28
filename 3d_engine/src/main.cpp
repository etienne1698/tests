#include <dawn/dawn_proc.h>
#include <dawn/webgpu_cpp.h>
#include <dawn/webgpu.h>

#include <iostream>

int main()
{
    // Initialisation de Dawn
    std::cout << "Initialisation de Dawn..." << std::endl;

    // Charger les fonctions de Dawn
    dawnProcSetProcs(&dawn_native::GetProcs());

    // Exemple simple de création d'un appareil
    WGPUInstance instance = wgpuCreateInstance(nullptr);
    if (!instance)
    {
        std::cerr << "Échec de la création de l'instance WebGPU." << std::endl;
        return -1;
    }

    std::cout << "Dawn initialisé avec succès !" << std::endl;

    // Nettoyage
    wgpuInstanceRelease(instance);

    return 0;
}
